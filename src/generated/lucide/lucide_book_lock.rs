use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H10" ></ path > < path d = "M20 15v7H6.5a2.5 2.5 0 0 1 0-5H20" ></ path > < rect width = "8" x = "12" height = "5" y = "6" rx = "1" ></ rect > < path d = "M18 6V4a2 2 0 1 0-4 0v2" ></ path > < / > } } pub const LUCIDE_BOOK_LOCK : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;