use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "17" r = "1" ></ circle > < path d = "M3 7v6h6" ></ path > < path d = "M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13" ></ path > < / > } } pub const LUCIDE_UNDO_DOT : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;