use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < rect height = "18" y = "3" x = "3" width = "18" rx = "2" ></ rect > < path d = "M16 8.9V7H8l4 5-4 5h8v-1.9" ></ path > < / > } } pub const LUCIDE_SQUARE_SIGMA : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("width" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("stroke-linejoin" , "round")] } ;