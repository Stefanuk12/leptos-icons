use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" ></ path > < path d = "M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" ></ path > < path d = "M6 8h2" ></ path > < path d = "M6 12h2" ></ path > < path d = "M16 8h2" ></ path > < path d = "M16 12h2" ></ path > < / > } } pub const LUCIDE_BOOK_OPEN_TEXT : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24")] } ;