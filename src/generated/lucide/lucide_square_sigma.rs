use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect width = "18" rx = "2" x = "3" y = "3" height = "18" ></ rect > < path d = "M16 8.9V7H8l4 5-4 5h8v-1.9" ></ path > < / > } } pub const LUCIDE_SQUARE_SIGMA : Path = Path { path : icon_path , props : & [("width" , "24") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("fill" , "none")] } ;