use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M18 6V4a2 2 0 1 0-4 0v2" ></ path > < path d = "M20 15v6a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20" ></ path > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H10" ></ path > < rect y = "6" height = "5" rx = "1" x = "12" width = "8" ></ rect > < / > } } pub const LUCIDE_BOOK_LOCK : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor")] } ;