use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" ry = "2" height = "18" width = "18" x = "3" y = "3" ></ rect > < path d = "M16 8h.01" ></ path > < path d = "M12 12h.01" ></ path > < path d = "M8 16h.01" ></ path > < / > } } pub const LUCIDE_DICE_3 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("fill" , "none") , ("stroke-linecap" , "round") , ("width" , "24")] } ;