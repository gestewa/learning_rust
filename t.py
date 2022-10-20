from collections import Counter

class Solution:
    def containsNearbyDuplicate(self, nums, k):
        # nums[i] == nums[j] && abs(i=j) <= k
        # there are two numbers which are equal to eachother
        # these two numbers are within k of eachother in position in the array
        # [1,2,3,1], k = 3
        #  0 1 2 3
        # abs(0-3) = 3 <= k = 3
        # [1,2,3,1,2,3], k = 2
        #  0 1 2 3 4 5
        #  abs(0-3)=3 is not <= 2 => false
        
        # sliding window of size k will result in a solution of runtime k*len(nums)
        # In the worst case, we have to look at every item because the nearby duplicates could exist anywhere in the array
        # so the best solution will have a runtime of at least O(len(nums))
        
        # can we do better than O(k*len(nums))??
        # can we add some sort of data structure to quickly search so it can become constant time lookup within the window
        # what if we had a hash table of things within our k-length window of our current pointer?
        
        # if k is 0 there are no two DISTINCT indicies that can be equal
        if k == 0:
            return False
        
        # define the first window
        window = Counter(nums[0:k+1])
        # If the initial valid window has a duplicate return true 
        if len(list(window))<k+1:
            return True
        
        for idx,num in enumerate(nums[k+1:]):
            # update the window
            numToRemove = nums[idx-k]
            window -= Counter(`numToRemove`=1)
            
            numToAdd = nums[idx]
            window += Counter(`numToAdd`=1)
            print(window)
            # check if the there is a duplicate of the current element in the window
            if num in window:
                return True
            
        return False

print(Solution().containsNearbyDuplicate([1,2,3,1,2,3],2))
