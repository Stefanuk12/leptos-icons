use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect y = "3" rx = "2" width = "18" x = "3" height = "18" ></ rect > < path d = "M16 8.9V7H8l4 5-4 5h8v-1.9" ></ path > < / > } } pub const LUCIDE_SIGMA_SQUARE : Path = Path { path : icon_path , props : & [("stroke-width" , "2") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("fill" , "none") , ("stroke" , "currentColor")] } ;