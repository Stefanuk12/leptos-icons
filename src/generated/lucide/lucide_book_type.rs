use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 13h4" ></ path > < path d = "M12 6v7" ></ path > < path d = "M16 8V6H8v2" ></ path > < path d = "M4 19.5v-15A2.5 2.5 0 0 1 6.5 2H19a1 1 0 0 1 1 1v18a1 1 0 0 1-1 1H6.5a1 1 0 0 1 0-5H20" ></ path > < / > } } pub const LUCIDE_BOOK_TYPE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("width" , "24")] } ;