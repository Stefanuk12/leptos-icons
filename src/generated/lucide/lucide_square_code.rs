use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" y = "3" rx = "2" height = "18" x = "3" ></ rect > < path d = "m10 10-2 2 2 2" ></ path > < path d = "m14 14 2-2-2-2" ></ path > < / > } } pub const LUCIDE_SQUARE_CODE : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;