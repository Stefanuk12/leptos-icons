use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "m15 11-1 9" ></ path > < path d = "m19 11-4-7" ></ path > < path d = "M2 11h20" ></ path > < path d = "m3.5 11 1.6 7.4a2 2 0 0 0 2 1.6h9.8a2 2 0 0 0 2-1.6l1.7-7.4" ></ path > < path d = "M4.5 15.5h15" ></ path > < path d = "m5 11 4-7" ></ path > < path d = "m9 11 1 9" ></ path > < / > } } pub const LUCIDE_SHOPPING_BASKET : Path = Path { path : icon_path , props : & [("viewBox" , "0 0 24 24") , ("stroke-linejoin" , "round") , ("stroke" , "currentColor") , ("stroke-width" , "2") , ("width" , "24") , ("height" , "24") , ("stroke-linecap" , "round") , ("fill" , "none") , ("xmlns" , "http://www.w3.org/2000/svg")] } ;