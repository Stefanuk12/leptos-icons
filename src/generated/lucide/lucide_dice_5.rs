use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" y = "3" x = "3" width = "18" rx = "2" height = "18" ></ rect > < path d = "M16 8h.01" ></ path > < path d = "M8 8h.01" ></ path > < path d = "M8 16h.01" ></ path > < path d = "M16 16h.01" ></ path > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DICE_5 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke" , "currentColor") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("fill" , "none")] } ;