use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M12 7v6" ></ path > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20" ></ path > < path d = "M9 10h6" ></ path > < / > } } pub const LUCIDE_BOOK_PLUS : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2")] } ;