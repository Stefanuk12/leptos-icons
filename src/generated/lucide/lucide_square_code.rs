use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M10 9.5 8 12l2 2.5" ></ path > < path d = "m14 9.5 2 2.5-2 2.5" ></ path > < rect rx = "2" width = "18" y = "3" x = "3" height = "18" ></ rect > < / > } } pub const LUCIDE_SQUARE_CODE : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24")] } ;