#TODO: Revisit
class Solution:
    def findAllRecipes(self, recipes: List[str], ingredients: List[List[str]], supplies: List[str]) -> List[str]:
        available = set()
        for i in supplies:
            available.add(i)
        recipeQ = deque(range(len(recipes)))

        created: List[str]  = []
        last = -1

        while len(available) > last:
            last = len(available)
            qsize = len(recipeQ)
            while qsize > 0:
                qsize -=1
                recipeidx = recipeQ.popleft()
                if all(
                    ingredient in available
                    for ingredient in ingredients[recipeidx]
                ):
                    available.add(recipes[recipeidx])
                    created.append(recipes[recipeidx])
                else:
                    recipeQ.append(recipeidx)
        return created
        

