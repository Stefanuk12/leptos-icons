use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M2 12a5 5 0 0 0 5 5 8 8 0 0 1 5 2 8 8 0 0 1 5-2 5 5 0 0 0 5-5V7h-5a8 8 0 0 0-5 2 8 8 0 0 0-5-2H2Z" ></ path > < path d = "M6 11c1.5 0 3 .5 3 2-2 0-3 0-3-2Z" ></ path > < path d = "M18 11c-1.5 0-3 .5-3 2 2 0 3 0 3-2Z" ></ path > < / > } } pub const LUCIDE_VENETIAN_MASK : Path = Path { path : icon_path , props : & [("fill" , "none") , ("stroke-width" , "2") , ("stroke" , "currentColor") , ("xmlns" , "http://www.w3.org/2000/svg") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round")] } ;