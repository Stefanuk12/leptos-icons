use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" ></ path > < path d = "M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" ></ path > < / > } } pub const LUCIDE_BOOK_OPEN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24")] } ;