use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" ry = "2" y = "3" width = "18" height = "18" rx = "2" ></ rect > < path d = "M16 8h.01" ></ path > < path d = "M16 12h.01" ></ path > < path d = "M16 16h.01" ></ path > < path d = "M8 8h.01" ></ path > < path d = "M8 12h.01" ></ path > < path d = "M8 16h.01" ></ path > < / > } } pub const LUCIDE_DICE_6 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;