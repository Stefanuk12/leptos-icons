use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" ry = "2" x = "3" rx = "2" width = "18" height = "18" ></ rect > < path d = "m15 9-6 6" ></ path > < path d = "m9 9 6 6" ></ path > < / > } } pub const LUCIDE_SQUARE_X : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("fill" , "none") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24")] } ;