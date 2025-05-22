-- Read the natal payload from file
local file = io.open("natal_payload.json", "r")
local payload = file:read("*all")
file:close()

-- Set up the request
request = function()
    local headers = {}
    headers["Content-Type"] = "application/json"
    return wrk.format("POST", "/api/chart/natal", headers, payload)
end 