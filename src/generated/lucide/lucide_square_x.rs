use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" width = "18" y = "3" ry = "2" height = "18" x = "3" ></ rect > < path d = "m15 9-6 6" ></ path > < path d = "m9 9 6 6" ></ path > < / > } } pub const LUCIDE_SQUARE_X : Path = Path { path : icon_path , props : & [("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("fill" , "none") , ("height" , "24") , ("stroke" , "currentColor") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;