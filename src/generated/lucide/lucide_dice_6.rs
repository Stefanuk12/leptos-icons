use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" ry = "2" width = "18" x = "3" y = "3" rx = "2" ></ rect > < path d = "M16 8h.01" ></ path > < path d = "M16 12h.01" ></ path > < path d = "M16 16h.01" ></ path > < path d = "M8 8h.01" ></ path > < path d = "M8 12h.01" ></ path > < path d = "M8 16h.01" ></ path > < / > } } pub const LUCIDE_DICE_6 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;