use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 9.5 8 12l2 2.5" ></ path > < path d = "m14 9.5 2 2.5-2 2.5" ></ path > < rect height = "18" width = "18" rx = "2" y = "3" x = "3" ></ rect > < / > } } pub const LUCIDE_SQUARE_CODE : Path = Path { path : icon_path , props : & [("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;