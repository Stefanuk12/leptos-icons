use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" x = "3" height = "18" ry = "2" y = "3" rx = "2" ></ rect > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DICE_1 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor") , ("height" , "24") , ("width" , "24")] } ;