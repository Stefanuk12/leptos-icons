use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" ry = "2" x = "3" height = "18" y = "3" ></ rect > < path d = "M16 8h.01" ></ path > < path d = "M12 12h.01" ></ path > < path d = "M8 16h.01" ></ path > < / > } } pub const LUCIDE_DICE_3 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke-width" , "2") , ("fill" , "none") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round")] } ;