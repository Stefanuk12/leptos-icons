use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M6 3h12" ></ path > < path d = "M6 8h12" ></ path > < path d = "m6 13 8.5 8" ></ path > < path d = "M6 13h3" ></ path > < path d = "M9 13c6.667 0 6.667-10 0-10" ></ path > < / > } } pub const LUCIDE_INDIAN_RUPEE : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("fill" , "none") , ("height" , "24") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-linecap" , "round") , ("stroke" , "currentColor") , ("width" , "24")] } ;