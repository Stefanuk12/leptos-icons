use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" height = "18" y = "3" ry = "2" rx = "2" ></ rect > < path d = "M15 9h.01" ></ path > < path d = "M9 15h.01" ></ path > < / > } } pub const LUCIDE_DICE_2 : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;