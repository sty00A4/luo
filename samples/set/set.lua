__IMPLS = {
    Set = {
        __name = "Set",
        __add = function(self, other)
            local new = Set.copy(self)
            for k, v in pairs(other) do
                if v then new[k] = v end
            end
            return new
        end,
        __sub = function(self, other)
            local new = Set.copy(self)
            for k, v in pairs(other) do
                if v then new[k] = nil end
            end
            return new
        end,
        __eq = function(self, other)
            for k, v in pairs(other) do
                if self[k] ~= v then return false end
            end
            for k, v in pairs(self) do
                if other[k] ~= v then return false end
            end
            return true
        end,
    }
}
Set = {
    new = function(t)
        local set = {}
        for _, v in ipairs(t) do
            set[v] = true
        end
        setmetatable(set, __IMPLS.Set)
    end,
    from = function(t)
        setmetatable(t, __IMPLS.Set)
    end,
    copy = function(set)
        local new = Set.new{}
        for k, v in pairs(set) do
            new[k] = v
        end
        return new
    end,
}