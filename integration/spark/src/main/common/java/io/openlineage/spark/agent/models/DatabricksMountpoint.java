package io.openlineage.spark.agent.models;

import io.openlineage.client.shaded.com.fasterxml.jackson.annotation.JsonIgnoreProperties;
import io.openlineage.client.shaded.com.fasterxml.jackson.annotation.JsonProperty;
import lombok.Value;

@JsonIgnoreProperties(ignoreUnknown = true)
@Value
public class DatabricksMountpoint {

  public DatabricksMountpoint(
      @JsonProperty("mountPointString") String mountPoint,
      @JsonProperty("sourceString") String source) {
    this.mountPoint = mountPoint;
    this.source = source;
  }

  String mountPoint;
  String source;
}
