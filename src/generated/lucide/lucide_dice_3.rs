use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect x = "3" rx = "2" height = "18" width = "18" y = "3" ry = "2" ></ rect > < path d = "M16 8h.01" ></ path > < path d = "M12 12h.01" ></ path > < path d = "M8 16h.01" ></ path > < / > } } pub const LUCIDE_DICE_3 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("width" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24")] } ;