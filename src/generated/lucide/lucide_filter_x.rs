use leptos :: * ; use crate :: Path ; fn icon_path () -> Fragment { view ! { < > < path d = "M13.013 3H2l8 9.46V19l4 2v-8.54l.9-1.055" ></ path > < path d = "m22 3-5 5" ></ path > < path d = "m17 3 5 5" ></ path > < / > } } pub const LUCIDE_FILTER_X : Path = Path { path : icon_path , props : & [("stroke" , "currentColor") , ("width" , "24") , ("stroke-linecap" , "round") , ("xmlns" , "http://www.w3.org/2000/svg") , ("height" , "24") , ("fill" , "none") , ("stroke-width" , "2") , ("stroke-linejoin" , "round") , ("viewBox" , "0 0 24 24")] } ;