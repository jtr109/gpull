use anyhow::Result;

pub fn map_image_name(image: &str) -> Result<String> {
    // example:
    // original: k8s.gcr.io/sig-storage/csi-node-driver-registrar:v2.2.0
    // new: registry.aliyuncs.com/google_containers/csi-node-driver-registrar:v2.2.0
    let tail = image
        .split("/")
        .last()
        .ok_or(anyhow::anyhow!("invalid image name: {}", image))?;
    Ok(format!("registry.aliyuncs.com/google_containers/{}", tail))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_image_name() {
        let original = "k8s.gcr.io/sig-storage/csi-node-driver-registrar:v2.2.0";
        let expected = "registry.aliyuncs.com/google_containers/csi-node-driver-registrar:v2.2.0";
        assert_eq!(map_image_name(original).unwrap(), expected);
    }
}
