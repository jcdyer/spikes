package go2020

import (
	"go.mongodb.org/mongo-driver/bson/primitive"
)

func GetID() string {
	return primitive.NewObjectID().Hex()
}
