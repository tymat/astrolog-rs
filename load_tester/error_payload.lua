-- Read the error payload from file
local file = io.open("error_payload.json", "r")
local payload = file:read("*all")
file:close()

-- Set up the request
request = function()
    local headers = {}
    headers["Content-Type"] = "application/json"
    return wrk.format("POST", "/api/chart/transit", headers, payload)
end

-- Optional: Add response handling
response = function(status, headers, body)
    if status ~= 500 then
        print("Unexpected status: " .. status)
    end
end 