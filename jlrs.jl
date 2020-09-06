module Jlrs
using Base.StackTraces

struct TracedException
    exc
    stacktrace::StackTrace
end

function runasync(func, wakeptr, args...)
    result = func(args...)
    
    # wakerust is set by jlrs
    ccall(wakerust, Cvoid, (Ptr{Cvoid}, ), wakeptr)
    result
end

function asynccall(func, wakeptr, args...)
    Base.Threads.@spawn runasync(func, wakeptr, args...)
end

function tracingcall(func)
    function wrapper(args...)
        try
            func(args...)
        catch exc
            for s in stacktrace(catch_backtrace(), true)
                println(stderr, s)
            end

            rethrow(exc)
        end
    end

    wrapper
end

function attachstacktrace(func)
    function wrapper(args...)
        try
            func(args...)
        catch exc
            st = stacktrace(catch_backtrace(), true)
            throw(TracedException(exc, st))
        end
    end

    wrapper
end
end