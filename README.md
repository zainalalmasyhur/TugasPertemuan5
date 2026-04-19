# API Automation Testing - Books

## Base URL
`https://fakerestapi.azurewebsites.net/api/v1/`

---

## Tools
- Katalon Studio
- Groovy
- GitHub

---

## Endpoints

### 1. Get All Books
#### GET /Books
- **Description**: Retrieve all books data
- **Request**:
  - Method: GET
  - URL: `/api/v1/Books`
- **Response**:
  - **200 OK**
    ```json
    [
      {
        "id":1,
        "title":"Book 1",
        "description":"Lorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\n",
        "pageCount":100,
        "excerpt":"Lorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\n",
        "publishDate":"2026-04-18T11:12:55.9703493+00:00"
      },
    ]
    ```

---

### 2. Get Book by ID
#### GET /Books/{id}
- **Description**: Retrieve book detail by ID
- **Request**:
  - Method: GET
  - URL: `/api/v1/Books/{id}`
- **Parameters**:
  - id (integer)
- **Response**:
  - **200 OK**
    ```json
    {
      "id":1,
      "title":"Book 1",
      "description":"Lorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\n",
      "pageCount":100,
      "excerpt":"Lorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\nLorem lorem lorem. Lorem lorem lorem. Lorem lorem lorem.\n",
      "publishDate":"2026-04-18T11:12:55.9703493+00:00"
     }
    ```

---

### 3. Create Book
#### POST /Books
- **Description**: Create new book data
- **Request**:
  - Method: POST
  - URL: `/api/v1/Books`
  - Headers:
    - Content-Type: application/json
  - Body:
    ```json
    {
      "title": "Buku Hisana",
      "description": "Buku ayam",
      "pageCount": 500,
      "excerpt": "HAaaaaa",
      "publishDate": "2026-04-19T09:40:21.160Z"
    }
    ```
- **Response**:
  - **200 OK / 201 Created**

---

### 4. Update Book
#### PUT /Books/{id}
- **Description**: Update book data
- **Request**:
  - Method: PUT
  - URL: `/api/v1/Books/{id}`
  - Body:
    ```json
    {
      "id": 1,
      "title": "Buku Hisana",
      "description": "Buku ayam",
      "pageCount": 500,
      "excerpt": "HAaaaaa",
      "publishDate": "2026-04-19T09:40:21.160Z"
    }
    ```
- **Response**:
  - **200 OK**

---

### 5. Delete Book
#### DELETE /Books/{id}
- **Description**: Delete book data
- **Request**:
  - Method: DELETE
  - URL: `/api/v1/Books/{id}`
- **Response**:
  - **200 OK**

---

## Test Scenarios

### ✅ Positive Test
- Get all books → status 200
- Get book by valid ID → data sesuai
- Create new book → berhasil
- Update book → data berubah
- Delete book → berhasil

---

## Test Structure (Katalon)
### Test Cases:
- TC_GET_All_Books
- TC_GET_Book_By_ID
- TC_POST_Create_Book
- TC_PUT_Update_Book
- TC_DELETE_Book

### Test Suite:
- TS_Books_API

---

## How to Run

1. Clone repository
2. Open project in Katalon Studio
3. Run Test Suite `TS_Books_API`

---

## Test Result

- All test cases executed
- Status: PASS ✅

---

## Notes
- Menggunakan parameterization untuk dynamic data
- Menggunakan verification status code & response body
- Automation dilakukan menggunakan Katalon Studio

---

## Author
Zainal Abidin
