use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" ry = "2" x = "3" y = "3" width = "18" height = "18" ></ rect > < path d = "M16 8h.01" ></ path > < path d = "M8 8h.01" ></ path > < path d = "M8 16h.01" ></ path > < path d = "M16 16h.01" ></ path > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DICE_5 : Path = Path { path : icon_path , props : & [("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24") , ("stroke" , "currentColor") , ("height" , "24") , ("stroke-width" , "2") , ("viewBox" , "0 0 24 24")] } ;