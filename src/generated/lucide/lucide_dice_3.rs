use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" width = "18" height = "18" y = "3" rx = "2" ry = "2" ></ rect > < path d = "M16 8h.01" ></ path > < path d = "M12 12h.01" ></ path > < path d = "M8 16h.01" ></ path > < / > } } pub const LUCIDE_DICE_3 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linecap" , "round") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("height" , "24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;