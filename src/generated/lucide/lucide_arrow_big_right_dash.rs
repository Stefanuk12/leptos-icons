use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 9v6" ></ path > < path d = "M9 9h3V5l7 7-7 7v-4H9V9z" ></ path > < / > } } pub const LUCIDE_ARROW_BIG_RIGHT_DASH : Path = Path { path : icon_path , props : & [("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-linecap" , "round") , ("stroke-width" , "2") , ("width" , "24") , ("viewBox" , "0 0 24 24") , ("stroke" , "currentColor") , ("stroke-linejoin" , "round")] } ;