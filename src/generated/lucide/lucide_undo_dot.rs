use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < circle cx = "12" cy = "17" r = "1" ></ circle > < path d = "M3 7v6h6" ></ path > < path d = "M21 17a9 9 0 0 0-9-9 9 9 0 0 0-6 2.3L3 13" ></ path > < / > } } pub const LUCIDE_UNDO_DOT : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2")] } ;