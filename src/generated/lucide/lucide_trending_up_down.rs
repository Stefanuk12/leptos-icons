use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M14.828 14.828 21 21" ></ path > < path d = "M21 16v5h-5" ></ path > < path d = "m21 3-9 9-4-4-6 6" ></ path > < path d = "M21 8V3h-5" ></ path > < / > } } pub const LUCIDE_TRENDING_UP_DOWN : Path = Path { path : icon_path , props : & [("fill" , "none") , ("height" , "24") , ("viewBox" , "0 0 24 24") , ("width" , "24") , ("stroke" , "currentColor") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("stroke-width" , "2") , ("stroke-linejoin" , "round")] } ;