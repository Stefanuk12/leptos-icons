use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" y = "3" rx = "2" ry = "2" width = "18" x = "3" ></ rect > < path d = "M16 8h.01" ></ path > < path d = "M8 8h.01" ></ path > < path d = "M8 16h.01" ></ path > < path d = "M16 16h.01" ></ path > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DICE_5 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("fill" , "none") , ("width" , "24") , ("stroke-linejoin" , "round") , ("height" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-width" , "2")] } ;