public class heap1 {
    private final int[] heap;
    private int size;

    public heap1(int capacity) {
        heap = new int[capacity];
        size = 0;
    }

    public heap1(int capacity, int[] arr) {
        heap = new int[capacity];
        size = arr.length;
        System.arraycopy(arr, 0, heap, 0, size);
        buildHeap();
    }

    public void heapify(int[] arr, int n, int i) {
        int largest = i;
        int left = 2 * i + 1;
        int right = 2 * i + 2;

        if (left < n && arr[left] > arr[largest]) {
            largest = left;
        }

        if (right < n && arr[right] > arr[largest]) {
            largest = right;
        }

        if (largest != i) {
            int temp = arr[i];
            arr[i] = arr[largest];
            arr[largest] = temp;

            heapify(arr, n, largest);
        }
    }

    private void buildHeap() {
        for (int i = size / 2 - 1; i >= 0; i--) {
            heapify(heap, size, i);
        }
    }

    public void add(int value) {
        if (size == heap.length) {
            throw new IllegalStateException("heap is full");
        }
        heap[size] = value;
        size++;
        heapifyUp(size - 1);
    }

    public void heapifyUp(int index) {
        int parent = (index - 1) / 2;

        while (index > 0 && heap[index] > heap[parent]) {
            int temp = heap[parent];
            heap[parent] = heap[index];
            heap[index] = temp;

            index = parent;
            parent = (index - 1) / 2;
        }
    }

    public void printHeap() {
        for (int i = 0; i < size; i++) {
            System.out.print(heap[i] + " ");
        }
        System.out.println("");
    }

    public static void main(String args[]) {
        heap1 maxHeap1 = new heap1(100);
        maxHeap1.add(5);
        maxHeap1.add(10);
        maxHeap1.add(50);
        maxHeap1.add(30);
        maxHeap1.add(80);
        maxHeap1.add(46);
        maxHeap1.add(7);
        maxHeap1.printHeap();

        int[] arr = { 20, 23, 50, 41, 56, 75, 89, 52, 15 };
        heap1 mHeap1 = new heap1(100, arr);
        mHeap1.printHeap();
        mHeap1.add(50);
        mHeap1.printHeap();
    }
}