#!/bin/bash
# Docker Hub Push Script for Chilli Images

set -e

REGISTRY="${REGISTRY:-docker.io}"
NAMESPACE="${NAMESPACE:-ctkqiang}"
VERSION="${VERSION:-latest}"

echo "🐳 Publishing Chilli Docker Images"
echo "=================================="

# Backend API Image
BACKEND_IMAGE="${REGISTRY}/${NAMESPACE}/chilli:${VERSION}"
echo ""
echo "📦 Backend API: $BACKEND_IMAGE"
docker push "$BACKEND_IMAGE"
echo "✅ Backend published successfully"

# Portal Frontend Image
PORTAL_IMAGE="${REGISTRY}/${NAMESPACE}/chilli-portal:${VERSION}"
echo ""
echo "📦 Portal Frontend: $PORTAL_IMAGE"
docker push "$PORTAL_IMAGE"
echo "✅ Portal published successfully"

echo ""
echo "=================================="
echo "🎉 All images published successfully!"
echo ""
echo "Images available at:"
echo "  - $BACKEND_IMAGE"
echo "  - $PORTAL_IMAGE"
echo ""
echo "Usage:"
echo "  Backend:  docker pull $BACKEND_IMAGE"
echo "  Portal:   docker pull $PORTAL_IMAGE"
