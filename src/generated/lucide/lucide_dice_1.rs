use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect rx = "2" y = "3" height = "18" ry = "2" width = "18" x = "3" ></ rect > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DICE_1 : Path = Path { path : icon_path , props : & [("height" , "24") , ("stroke" , "currentColor") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24")] } ;