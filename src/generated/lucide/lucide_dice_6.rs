use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" width = "18" height = "18" x = "3" y = "3" rx = "2" ></ rect > < path d = "M16 8h.01" ></ path > < path d = "M16 12h.01" ></ path > < path d = "M16 16h.01" ></ path > < path d = "M8 8h.01" ></ path > < path d = "M8 12h.01" ></ path > < path d = "M8 16h.01" ></ path > < / > } } pub const LUCIDE_DICE_6 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24")] } ;