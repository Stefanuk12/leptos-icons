use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" width = "18" x = "3" rx = "2" ry = "2" y = "3" ></ rect > < path d = "m15 9-6 6" ></ path > < path d = "m9 9 6 6" ></ path > < / > } } pub const LUCIDE_SQUARE_X : Path = Path { path : icon_path , props : & [("width" , "24") , ("height" , "24") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round")] } ;