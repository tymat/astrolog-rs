#!/usr/bin/env ruby
require 'json'
require 'typhoeus'

def transit_payload
  payload = {
    natal_date: "1977-10-24T04:56:00Z",
    transit_date: "2025-06-01T17:21:03-10:00",
    latitude: 14.6486,
    longitude: 121.0508,
    house_system: "placidus",
    ayanamsa: "tropical"
  }.to_json
end

def synastry_payload
  payload = {
    chart1: {
      date: "1977-10-24T04:56:00Z",
      latitude: 14.6488,
      longitude: 121.0509,
      house_system: "placidus",
      ayanamsa: "tropical"
    },
    chart2: {
      date: "1996-01-06T12:00:00Z",
      latitude: 36.66833,
      longitude: 116.99722,
      house_system: "placidus",
      ayanamsa: "tropical"
    }
  }.to_json
end

def chart_payload
  payload = {
    date: "1977-10-24T12:56:00+08:00",
    latitude: 14.6486,
    longitude: 121.0508,
    house_system: "placidus",
    ayanamsa: "tropical",
    include_minor_aspects: false,
    transit: {
      date: "2025-06-01T17:03:50-10:00",
      latitude: 19.49,
      longitude: -155.99
    }
  }.to_json
end


def get_chart
  a = Typhoeus::Request.post("http://localhost:4008/api/chart", body: chart_payload, headers: {"Content-Type": "application/json"}).response_body
  f = File.new("svg/natal.svg", 'w')
  f.puts JSON.parse(a)["svg_chart"]
  f.close
end

def get_synastry
  a = Typhoeus::Request.post("http://localhost:4008/api/chart/synastry", body: synastry_payload, headers: {"Content-Type": "application/json"}).response_body
  f = File.new("svg/synastry.svg", 'w')
  f.puts JSON.parse(a)["svg_chart"]
  f.close
end

def get_transit
  a = Typhoeus::Request.post("http://localhost:4008/api/chart/transit", body: transit_payload, headers: {"Content-Type": "application/json"}).response_body
  f = File.new("svg/transit.svg", 'w')
  f.puts JSON.parse(a)["svg_chart"]
  f.close
end


get_chart
get_transit
get_synastry
