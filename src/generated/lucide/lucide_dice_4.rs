use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" ry = "2" x = "3" height = "18" y = "3" ></ rect > < path d = "M16 8h.01" ></ path > < path d = "M8 8h.01" ></ path > < path d = "M8 16h.01" ></ path > < path d = "M16 16h.01" ></ path > < / > } } pub const LUCIDE_DICE_4 : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("width" , "24") , ("stroke-width" , "2") , ("height" , "24") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round")] } ;