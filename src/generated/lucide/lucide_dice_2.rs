use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" ry = "2" y = "3" height = "18" rx = "2" x = "3" ></ rect > < path d = "M15 9h.01" ></ path > < path d = "M9 15h.01" ></ path > < / > } } pub const LUCIDE_DICE_2 : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;