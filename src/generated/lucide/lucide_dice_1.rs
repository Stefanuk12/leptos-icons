use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect ry = "2" height = "18" y = "3" rx = "2" width = "18" x = "3" ></ rect > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DICE_1 : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("width" , "24") , ("fill" , "none") , ("height" , "24") , ("stroke-width" , "2") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;