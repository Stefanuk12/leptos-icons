use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 6V4a2 2 0 1 0-4 0v2" ></ path > < path d = "M20 15v6a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20" ></ path > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H10" ></ path > < rect width = "8" x = "12" height = "5" rx = "1" y = "6" ></ rect > < / > } } pub const LUCIDE_BOOK_LOCK : Path = Path { path : icon_path , props : & [("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;