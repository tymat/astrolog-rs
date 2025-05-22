-- Read the synastry payload from file
local file = io.open("synastry_payload.json", "r")
local payload = file:read("*all")
file:close()

-- Set up the request
request = function()
    local headers = {}
    headers["Content-Type"] = "application/json"
    return wrk.format("POST", "/api/chart/synastry", headers, payload)
end 