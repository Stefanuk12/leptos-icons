use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" rx = "2" ry = "2" width = "18" x = "3" y = "3" ></ rect > < path d = "M12 12h.01" ></ path > < / > } } pub const LUCIDE_DICE_1 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round")] } ;