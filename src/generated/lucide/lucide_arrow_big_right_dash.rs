use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M5 9v6" ></ path > < path d = "M9 9h3V5l7 7-7 7v-4H9V9z" ></ path > < / > } } pub const LUCIDE_ARROW_BIG_RIGHT_DASH : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("fill" , "none") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("height" , "24") , ("width" , "24")] } ;